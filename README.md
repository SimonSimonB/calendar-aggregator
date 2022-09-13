# Launching locally
1. Build frontend with `npm run build --dev` in `frontend/`.
2. Build and start backend with `FRONTEND_PATH=<PATH_TO_REPO>/calendar-aggregator/frontend/build/ cargo run` in `backend/`. (You must have Rust installed; [here are instructions](https://www.rust-lang.org/tools/install).)
3. Navigate to `http://127.0.0.1:8000`.