from fastapi import FastAPI
from starlette.middleware.cors import CORSMiddleware
from modules.downloads.ytdwnld import router
app = FastAPI(
    title="Microservice API"
)

# Configure CORS
origins = [
    "http://localhost:3000",
    "*"
    # Add other origins if needed
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


app.include_router(router)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)