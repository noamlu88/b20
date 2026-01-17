from flask import jsonify, request
from models.customer import Customer

def get_customers():
    customers = Customer.get_all()
    return jsonify([dict(row) for row in customers])

def create_customer():
    data = request.get_json()
    try:
        Customer.create(data['full_name'], data['email'], data['phone'])
        return '', 201
    except Exception as e:
        return '', 500

def update_customer(id):
    data = request.get_json()
    try:
        Customer.update(id, data['full_name'], data['email'], data['phone'])
        return '', 200
    except Exception as e:
        return '', 500

def delete_customer(id):
    try:
        Customer.delete(id)
        return '', 200
    except Exception as e:
        return '', 500
