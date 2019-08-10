import React, {Component} from 'react';
import {Input, Dropdown, Checkbox} from 'semantic-ui-react'
import '../App.css'

import {loadResult} from "../lib/ioManager";


const dists_methods = [
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
        text: "Rust (legacy)",
        key: 2,
        value: "rust_legacy"
    },
    {
        text: "Rust (modern)",
        key: 3,
        value: "rust"
    }
];


const algorithms = [
    {
        text: "Stupid",
        key: 1,
        value: "stupid"
    },
    {
        text: "Ordered",
        key: 2,
        value: "ordered"
    },
    {
        text: "Generic",
        key: 3,
        value: "generic"
    },
    {
        text: "Opt",
        key: 4,
        value: "opt"
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
            dists_method: value,
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

    onChangeAlgorithm = (e, data) => {
        const value = data.value;
        this.props.changeConfig({
            solve_algorithm: value,
        });
    };


    onChangeFinalRoute = () => {
        this.props.changeConfig({
            final_route: !this.props.config.final_route,
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
            Algorithm: <Dropdown placeholder='Algorithm'
                                 defaultValue={this.props.config.solve_algorithm}
                                 onChange={this.onChangeAlgorithm}
                                 fluid selection options={algorithms}/>
            Routing backend: <Dropdown placeholder='Routing backend'
                                       defaultValue={this.props.config.dists_method}
                                       onChange={this.onChangeBackend}
                                       fluid selection options={dists_methods}/>
            Number of candidates per event: <Input placeholder="Clipping"
                                                   fluid
                                                   defaultValue={this.props.config.clipping}
                                                   onChange={this.onClippingChange}/>
            Enable final route: <Checkbox fitted toggle onChange={this.onChangeFinalRoute}
                                          checked={this.props.config.final_route}/>
        </div>)
    }
}