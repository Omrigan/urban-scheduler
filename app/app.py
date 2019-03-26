from flask import Flask, render_template
from flask import request, jsonify

from lib import pretty_json, cities
from flask_cors import CORS
from exceptions import USException
import traceback

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
    ordered_events = data.get("ordered_events")
    # job = None
    schedule = None
    # schedule = job.predict_ordered(ordered_events)

    result = {
        "schedule": schedule,
        "center": cities['moscow']['center'],
        # "report": job.report()
    }
    return jsonify(result)
