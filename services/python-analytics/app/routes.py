from fastapi import APIRouter
from app.models import TransactionInput, RiskResult
from app.fraud import score_transaction

router = APIRouter()

@router.post("/score", response_model=RiskResult)
def score(tx: TransactionInput):
    return score_transaction(tx)
