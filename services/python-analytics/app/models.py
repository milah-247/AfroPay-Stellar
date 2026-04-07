from pydantic import BaseModel
from typing import Optional

class TransactionInput(BaseModel):
    tx_id: str
    user_id: str
    amount: float
    asset_code: str
    destination: str
    source_country: Optional[str] = None
    destination_country: Optional[str] = None

class RiskResult(BaseModel):
    tx_id: str
    risk_score: float       # 0.0 (safe) to 1.0 (high risk)
    flagged: bool
    reasons: list[str]
