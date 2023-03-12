from .app import App
import uvicorn

if __name__ == "__main__":
    app = App()
    uvicorn.run(app, host="0.0.0.0")
