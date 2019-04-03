// @flow

import React, {Component, Fragment} from 'react'
import {Map, TileLayer, Marker, Popup} from 'react-leaflet'
import L from 'leaflet';
import {Message, Accordion, Icon} from "semantic-ui-react";
import {Event} from "./Event";
import {OptionsContext} from "../lib/api";

import 'bootstrap/dist/css/bootstrap.min.css';

delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
    iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
    iconUrl: require('leaflet/dist/images/marker-icon.png'),
    shadowUrl: require('leaflet/dist/images/marker-shadow.png')
});

const MyPopupMarker = ({content, position}) => (
    <Marker position={position}>
        <Popup>{content}</Popup>
    </Marker>
);

const MyMarkersList = ({schedule}) => {
    const markers = schedule.map((x, i) => ({
        key: i,
        name: i,
        position: [x["location"]['lat'], x["location"]['lng']],
        content: x["brand"]

    }));

    const items = markers.map(({key, ...props}) => (
        <MyPopupMarker key={key} {...props} />
    ));
    return <Fragment>{items}</Fragment>
};

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