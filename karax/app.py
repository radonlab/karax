from fastapi import FastAPI
from .bluetooth.router import router

app = FastAPI()

app.include_router(router)
