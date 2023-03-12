FROM node:19-alpine3.15 as FRONTEND_BUILD
WORKDIR /app/frontend/
COPY frontend/package*.json .
RUN npm install
COPY frontend/ .
RUN npm run build

FROM python:3.11.2-bullseye as BACKEND_BUILD
WORKDIR /app/backend/

RUN pip install poetry

COPY backend/pyproject.toml .
COPY backend/poetry.lock .
RUN poetry config virtualenvs.create false
RUN poetry install --no-dev

COPY backend/ .

COPY --from=FRONTEND_BUILD /app/frontend/build /app/frontend/build
ENV FRONTEND_PATH=/app/frontend/build

ENTRYPOINT ["python", "-m", "calendar_aggregator.main"]