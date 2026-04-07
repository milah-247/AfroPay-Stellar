from fastapi import FastAPI
from app.routes import router

app = FastAPI(title="RemitX Fraud Detection")
app.include_router(router, prefix="/fraud")
