// @flow

import React, {Component, Fragment} from 'react'
import {Map, TileLayer, Marker, Popup} from 'react-leaflet'
import L from 'leaflet';

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
    return items
};

export default class ListOfMarkers extends Component<> {

    render() {
        return (
            <Map center={this.props.center} zoom={13} id="mapid">
                <TileLayer
                    attribution=''
                    url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />
                <MyMarkersList schedule={this.props.schedule}/>
            </Map>
        )
    }
}