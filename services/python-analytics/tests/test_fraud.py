import pytest
from app.fraud import score_transaction
from app.models import TransactionInput

def make_tx(**kwargs):
    defaults = dict(tx_id="tx1", user_id="u1", amount=100.0, asset_code="USDC", destination="GXXX")
    return TransactionInput(**{**defaults, **kwargs})

def test_safe_transaction():
    result = score_transaction(make_tx())
    assert result.flagged is False
    assert result.risk_score < 0.5

def test_large_amount_flagged():
    result = score_transaction(make_tx(amount=15000.0))
    assert result.risk_score >= 0.4
    assert any("Large" in r for r in result.reasons)

def test_high_risk_country_flagged():
    result = score_transaction(make_tx(destination_country="KP"))
    assert result.flagged is True
    assert result.risk_score >= 0.5

def test_round_number_adds_score():
    result = score_transaction(make_tx(amount=5000.0))
    assert any("Round" in r for r in result.reasons)

def test_score_capped_at_1():
    result = score_transaction(make_tx(amount=20000.0, destination_country="IR", source_country="KP"))
    assert result.risk_score <= 1.0
