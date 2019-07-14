import React, {Component} from 'react';
import {Input, Dropdown} from 'semantic-ui-react'
import '../App.css'

import {loadResult} from "../lib/ioManager";


const routingBackends = [
    {
        text: "Car (OSRM)",
        key: 1,
        value: "osrm"
    },
    {
        text: "Distance",
        key: 2,
        value: "dummy"
    },
    {
        text: "Public transport (HERE)",
        key: 3,
        value: "here"
    }
];

const solvers = [
    {
        text: "Python",
        key: 1,
        value: "python"
    },
    {
        text: "Rust",
        key: 2,
        value: "rust"
    }
];

export default class Config extends Component {

    constructor(props) {
        super(props);


    }


    onClippingChange = (e) => {
        const value = e.target.value;
        this.props.changeConfig({
            clipping: value
        });
    };

    onChangeBackend = (e, data) => {
        const value = data.value;
        this.props.changeConfig({
            routingBackend: value,
        });
    };

    onChangeCity = (e, data) => {
        const value = data.value;
        this.props.changeConfig({
            city: value,
        });
    };

    onChangeSolver = (e, data) => {
        const value = data.value;
        this.props.changeConfig({
            solver: value,
        });
    };


    render() {
        return (<div className="ui raised segment">
            <h4>Config</h4>
            City: <Dropdown placeholder='City'
                              defaultValue={this.props.config.city}
                              onChange={this.onChangeCity}
                              fluid selection options={this.props.cities}/>
            Solver: <Dropdown placeholder='Solver'
                              defaultValue={this.props.config.solver}
                              onChange={this.onChangeSolver}
                              fluid selection options={solvers}/>
            Routing backend: <Dropdown placeholder='Routing backend'
                                       defaultValue={this.props.config.routingBackend}
                                       onChange={this.onChangeBackend}
                                       fluid selection options={routingBackends}/>
            Number of candidates per event: <Input placeholder="Clipping"
                                                   fluid
                                                   defaultValue={this.props.config.clipping}
                                                   onChange={this.onClippingChange}/>
        </div>)
    }
}