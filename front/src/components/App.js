import React, {Component} from 'react';
// import logo from './logo.svg';
import '../App.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import {Job} from './Job';
import {Result} from './Result';
import 'leaflet/dist/leaflet.css'

// const sample_input = "[{\"type\": \"fixed_place\", \"location\": {\"lat\": 55.7494539, \"lng\": 37.62160470000001}, \"finish_time\": \"15:00\"}, {\"amenity\": \"cafe\", \"brand\": \"\u0434\u0430\u0431\u043b\u0431\u0438\", \"contact:email\": \"info@double-b.ru\", \"contact:facebook\": \"https://www.facebook.com/DoubleBCoffeeTea\", \"contact:instagram\": \"https://www.instagram.com/doublebcoffeetea\", \"contact:phone\": \"+7 968 7509961\", \"contact:telegram\": \"https://telegram.me/doublebdaily\", \"contact:website\": \"http://double-b.ru\", \"cuisine\": \"coffee_shop\", \"diet:vegetarian\": \"no\", \"drink:coffee\": \"yes\", \"name\": \"\u0414\u0430\u0431\u043b\u0431\u0438\", \"name:en\": \"Double B\", \"name:ru\": \"\u0414\u0430\u0431\u043b\u0431\u0438\", \"opening_hours\": \"Mo-Fr 08:00-23:00; Sa,Su 11:00-23:00\", \"takeaway\": \"yes\", \"location\": {\"lat\": \"55.7615389\", \"lng\": \"37.6315358\"}, \"categories\": [\"cafe\"], \"original_brand\": \"\u0414\u0430\u0431\u043b\u0431\u0438\"}, {\"name\": \"\u041c\u0443\u0437\u0435\u0439\u043d\u044b\u0439 \u043f\u0430\u0440\u043a\", \"location\": {\"lat\": 55.7589777, \"lng\": 37.62769979999999}, \"brand\": \"\u043c\u0443\u0437\u0435\u0439\u043d\u044b\u0439 \u043f\u0430\u0440\u043a\", \"categories\": [\"park\", \"point_of_interest\", \"establishment\"], \"additional_fields\": {}}, {\"name\": \"\u0412\u044b\u0441\u043e\u0442\u0430 5642\", \"location\": {\"lat\": 55.75592330000001, \"lng\": 37.62768570000001}, \"brand\": \"\u0432\u044b\u0441\u043e\u0442\u0430 5642\", \"categories\": [\"restaurant\", \"food\", \"point_of_interest\", \"establishment\"], \"additional_fields\": {}}, {\"type\": \"fixed_place\", \"start_time\": \"23:00\", \"location\": {\"lat\": 55.7494539, \"lng\": 37.62160470000001}}]\n";

// const sample_result = {
//     schedule: JSON.parse(sample_input),
//     center: [55.7494539, 37.62160470000001]
// };

class App extends Component {

    constructor(props) {
        super(props);
        this.state = {
            result: null
        }

    }

    updateResult = (result) => {
        this.setState({result: result})
    };
    componentDidMount() {
        console.log(process.env);
        document.title = process.env.REACT_APP_TITLE
    }

    render() {
        console.log(this.state.result);
        return (<div className="container">
            <div className="row">
                <div className="col-md-6 col-sm-12">
                    <h2>Job</h2>
                    <Job updateResult={this.updateResult}/>
                </div>

                <div className="col-md-6 col-sm-12">
                    <h2>Result</h2>
                    {this.state.result && <Result result={this.state.result}/>}
                </div>
            </div>

        </div>)
    }
}

export default App;
