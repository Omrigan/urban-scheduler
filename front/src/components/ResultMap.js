import React, {Component} from 'react';
import '../App.css';
import ListOfMarkers from './ListOfMarkers';
import ResultItemsList from './ResultItem';

// import HEREMap from 'react-here-maps'
import {Helmet} from "react-helmet";
// import '../vendor/mapsjs-core'
// import './vendor/mapsjs-service.js'


export default class ResultMap extends Component {
    constructor(props) {
        super(props);
        this.state = {}
    }

    render() {
        return (<React.Fragment>
            <div id="here-map" style={{width: '100%', height: '400px', background: 'grey'}}/>

        </React.Fragment>)
    }

    componentDidMount() {
        const H = window.H;
        const platform = new window.H.service.Platform({
            app_id: '3EGBDtqCF3N9erbelSMM',
            app_code: '7V_TmzxmRlBwas7-o5zizw',
            useHTTPS: true
        });

        const layer = platform.createDefaultLayers();
        const container = document.getElementById('here-map');


        const map = new window.H.Map(container, layer.normal.map, {
            center: {lat: this.props.center[0], lng: this.props.center[1]},
            zoom: 12,
        });
        this.state.map = map;

        const mapEvents = new H.mapevents.MapEvents(map);
        const behavior = new H.mapevents.Behavior(mapEvents);

        const markers = this.props.schedule.map(item => new H.map.Marker(item.location));
        this.state.map.addObjects(markers);

        const routeShape = this.props.final_route;

        if (routeShape) {
            let linestring = new H.geo.LineString();

            routeShape.forEach(function (point) {
                linestring.pushLatLngAlt(point[0], point[1]);
            });

            const routeLine = new H.map.Polyline(linestring, {
                style: {lineWidth: 10},
                arrows: {fillColor: 'white', frequency: 2, width: 0.8, length: 0.7}
            });
            this.state.map.addObject(routeLine);

        }


    }
}