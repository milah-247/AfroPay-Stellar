from app.models import TransactionInput, RiskResult

HIGH_RISK_COUNTRIES = {"KP", "IR", "SY"}
LARGE_AMOUNT_THRESHOLD = 10_000

def score_transaction(tx: TransactionInput) -> RiskResult:
    score = 0.0
    reasons = []

    if tx.amount > LARGE_AMOUNT_THRESHOLD:
        score += 0.4
        reasons.append("Large transaction amount")

    if tx.destination_country in HIGH_RISK_COUNTRIES:
        score += 0.5
        reasons.append(f"High-risk destination country: {tx.destination_country}")

    if tx.source_country in HIGH_RISK_COUNTRIES:
        score += 0.3
        reasons.append(f"High-risk source country: {tx.source_country}")

    # Round-number heuristic
    if tx.amount % 1000 == 0 and tx.amount > 0:
        score += 0.1
        reasons.append("Round-number amount")

    score = min(score, 1.0)
    flagged = score >= 0.5

    return RiskResult(tx_id=tx.tx_id, risk_score=round(score, 2), flagged=flagged, reasons=reasons)
