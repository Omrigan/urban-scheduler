from flask import Flask, render_template
from flask import request, jsonify

import lib
from flask_cors import CORS
from exceptions import USException
from logic.predictor import Predictor, RUST_URL
import traceback

import requests
from process.connector import get_db

db = get_db()

app = Flask(__name__)
CORS(app)


@app.errorhandler(USException)
def handle_us_exception(error):
    response = jsonify(error.to_dict())
    response.status_code = 400

    return response


@app.errorhandler(Exception)
def handle_any(error):
    dct = {
        'error_code': None,
        'error_name': error.__class__.__name__,
        'error_message': str(error)
    }
    traceback.print_exc()
    response = jsonify(dct)
    response.status_code = 500
    return response


@app.route('/predict', methods=["POST"])
def predict():
    data = request.get_json()
    if data.get("version") == 2:
        result = requests.post(RUST_URL, json=data)
        print("Proxy result:", result.text, flush=True)
        return result.text, result.status_code

    ordered_events = data.get("ordered_events")
    job = Predictor(data.get('config'))
    schedule = job.predict_ordered(ordered_events)
    result = {
        "schedule": schedule,
        "center": job.center(),
        "report": job.report(),
        "final_route": job.final_route
    }
    return jsonify(result)


@app.route('/get_params', methods=["GET"])
def get_params():
    cats = list(db.categories.find())
    for cat in cats:
        del cat['_id']

    return jsonify(cats)


@app.route('/cities', methods=["GET"])
def cities():
    return jsonify(lib.cities)
