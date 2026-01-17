from flask import jsonify, request
from models.account import Account

def get_accounts():
    accounts = Account.get_all()
    return jsonify([dict(row) for row in accounts])

def create_account():
    data = request.get_json()
    try:
        Account.create(data['customer_id'], data['account_number'], data['balance'], data['account_type'])
        return '', 201
    except Exception as e:
        return '', 500

def update_account(id):
    data = request.get_json()
    try:
        Account.update(id, data['account_number'], data['balance'], data['account_type'])
        return '', 200
    except Exception as e:
        return '', 500

def delete_account(id):
    try:
        Account.delete(id)
        return '', 200
    except Exception as e:
        return '', 500
