// @flow

import React, {Component} from 'react'
import L from 'leaflet';
import {Message} from "semantic-ui-react";

import 'bootstrap/dist/css/bootstrap.min.css';

delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
    iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
    iconUrl: require('leaflet/dist/images/marker-icon.png'),
    shadowUrl: require('leaflet/dist/images/marker-shadow.png')
});

export default class ListOfMarkers extends Component<> {

    render() {
        const report = this.props.report;
        return (
            <Message className="blue">

                <Message.Header><h4>Done</h4></Message.Header>
                Completed at {report.finish_time} <br/> <hr/>
                {report.stages.map((x, i) =>
                    (["Stage ",
                        x[0], " completed in ",
                        x[1], " milliseconds", <br/>]))}
                        <hr/>
                <h5>Config</h5>
                {Object.entries(report.config).map((x) =>
                    ([x[0], '=', x[1], <br/>]))}
                <hr/>
                <h5>Number of candidates per stage</h5>
                {Object.entries(report.numbers_of_candidates).map((x, i) =>
                    ([i, ": ", x[1], '; ']))}



            </Message>)

    }
}