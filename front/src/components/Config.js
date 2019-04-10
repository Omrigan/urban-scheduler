import React, {Component} from 'react';
import {Input, Dropdown} from 'semantic-ui-react'
import '../App.css'


const routingBackends = [
    {
        text: "OSRM - car",
        key: 1,
        value: "osrm"
    },
    {
        text: "Distance",
        key: 2,
        value: "dummy"
    },
    {
        text: "Here",
        key: 2,
        value: "here"
    }
];

const solvers = [
    {
        text: "Python",
        key: 2,
        value: "python"
    },
    {
        text: "Rust",
        key: 1,
        value: "rust"
    }
];

export default class Config extends Component {

    onClippingChange = (e) => {
        const value = e.target.value;
        this.props.onChangeConfig({
            clipping: value
        });
    };

    onChangeBackend = (e, data) => {
        const value = data.value;
        this.props.onChangeConfig({
            routingBackend: value,
        });
    };

    onChangeSolver = (e, data) => {
        const value = data.value;
        this.props.onChangeConfig({
            solver: value,
        });
    };


    render() {
        return (<div className="ui raised segment">
            <h4>Config</h4>
            Solver: <Dropdown placeholder='Solver'
                      defaultValue={this.props.config.solver}
                      onChange={this.onChangeSolver}
                      fluid selection options={solvers}/>
            Routing backend: <Dropdown placeholder='Routing backend'
                      defaultValue={this.props.config.routingBackend}
                      onChange={this.onChangeBackend}
                      fluid selection options={routingBackends}/>
            Number of candidates per event:  <Input placeholder="Clipping"
                   fluid
                   defaultValue={this.props.config.clipping}
                   onChange={this.onClippingChange}/>
        </div>)
    }
}