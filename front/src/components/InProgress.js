// @flow

import React, {Component} from 'react'
import L from 'leaflet';
import {Message} from "semantic-ui-react";

import 'bootstrap/dist/css/bootstrap.min.css';

export class InProgress extends Component<> {

    render() {
        return (
            <Message className="yellow">
                <Message.Header><h4>Predict is in progress</h4></Message.Header>
            </Message>)

    }
}