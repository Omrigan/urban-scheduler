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
                Completed at {report.finish_time} <br/>
                <hr/>
                {report.stages.map((x, i) =>
                    <React.Fragment key={i}>
                        Stage {x.name} completed in {x.timestamp} milliseconds <br />
                    </React.Fragment>)}

                <hr/>

                <h5>Config</h5>
                {Object.entries(this.props.config).map( (x, i) =>
                    <React.Fragment key={i}>
                        {x[0]}={x[1]} <br/>
                    </React.Fragment>)}
                <hr/>

                {/*<h5>Number of candidates per stage</h5>*/}
                {/*{Object.entries(report.numbers_of_candidates).map((x, i) =>*/}
                {/*      <React.Fragment key={i}>*/}
                {/*          {i}: {x[1]} <br/>*/}
                {/*    </React.Fragment>)}*/}
            </Message>)
    }
}