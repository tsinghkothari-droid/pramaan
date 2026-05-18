import json
from pathlib import Path


def render_order_summary() -> str:
    order = json.loads((Path(__file__).parent / "fixtures" / "order.json").read_text())
    return f"{order['id']}|{order['status']}|{order['total_cents']}"
