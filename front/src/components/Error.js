// @flow

import React, {Component} from 'react'
import L from 'leaflet';
import {Message} from "semantic-ui-react";

import 'bootstrap/dist/css/bootstrap.min.css';

export class Error extends Component<> {

    render() {
        const error = this.props.error;
        return (
            <Message className="red">

                <Message.Header><h4>{error.error_name || "Unknown error"}</h4></Message.Header>
                Code: {error.error_code} <br />
                Message: {error.error_message}

            </Message>)

    }
}