use std::env;

use sqlx::postgres::PgPool;

use crate::models::Topic;

pub struct Repository {
    connection_pool: PgPool,
}

impl Repository {
    pub async fn new() -> Result<Repository, Box<dyn std::error::Error>> {
        let connection_string = format!(
            "postgresql://{}:{}@{}:{}",
            &env::var("POSTGRES_USER")?,
            &env::var("POSTGRES_PASSWORD")?,
            &env::var("POSTGRES_HOST")?,
            &env::var("POSTGRES_PORT")?,
        );
        let repository = Repository {
            connection_pool: PgPool::connect(&connection_string).await?,
        };
        sqlx::migrate!().run(&repository.connection_pool).await?;
        Ok(repository)
    }

    pub async fn add_topic(self: &Self, name: &str, urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
        INSERT INTO topics (name)
        VALUES ($1)
      "#,
            name,
        )
        .execute(&self.connection_pool)
        .await?;
        
        let topic_id = sqlx::query!(
            r#"
        SELECT id FROM topics
        WHERE topics.name = $1
      "#,
            name,
        )
        .fetch_one(&self.connection_pool)
        .await?
        .id;
        
        for url in urls {
            sqlx::query!(
                r#"
            INSERT INTO urls (url)
            VALUES ($1)
            ON CONFLICT ON CONSTRAINT url_unique DO NOTHING
          "#,
                url,
            )
            .execute(&self.connection_pool)
            .await?;

            sqlx::query!(
                r#"
            INSERT INTO topics_urls (topic_id, url_id)
            VALUES ($1, (SELECT urls.id from urls WHERE urls.url = $2))
          "#,
                topic_id,
                url,
            )
            .execute(&self.connection_pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_topics(
        self: &Self,
    ) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
        SELECT id, name
        FROM topics
      "#,
        )
        .fetch_all(&self.connection_pool)
        .await?;
        
        let mut topics: Vec<Topic> = Vec::new();
        
        for rec in recs.iter() {
            let urls_recs = sqlx::query!(
                r#"
            SELECT url
            FROM urls
            WHERE EXISTS (SELECT * FROM topics_urls where topics_urls.url_id=urls.id)
          "#,
            )
            .fetch_all(&self.connection_pool)
            .await?;
            
            let urls = urls_recs.iter()
                .map(|urls_rec| urls_rec.url.clone())
                .collect();


            topics.push(Topic {
                id: rec.id as u32,
                name: rec.name.clone(),
                urls: urls,
            })
        }

        Ok(topics)
    }

    pub async fn get_topic_urls(
        self: &Self,
        topic_id: u32,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
        SELECT url
        FROM topics_urls
        INNER JOIN topics ON topics.id=topics_urls.topic_id
        INNER JOIN urls ON urls.id=topics_urls.url_id
        WHERE topics.id=$1
      "#,
            topic_id as i32,
        )
        .fetch_all(&self.connection_pool)
        .await?;

        Ok(recs
            .iter()
            .map(|record| record.url.clone())
            .collect())
    }
}
