import React, {Component} from 'react'
import {Map, TileLayer, Marker, Popup} from 'react-leaflet'
import L from 'leaflet';

delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
    iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
    iconUrl: require('leaflet/dist/images/marker-icon.png'),
    shadowUrl: require('leaflet/dist/images/marker-shadow.png')
});


export default class CoordinatesSelector extends Component<> {

    // handleLocationFound = (e) => {
    //
    //     console.log({
    //         hasLocation: true,
    //         latlng: e.latlng,
    //     })
    // };


    handleClick = (e) => {
        const latlng = e.latlng;
        this.props.onChange({location: latlng});
    };


    render() {
        const marker = this.props.location ? (
            <Marker position={this.props.location}>
                <Popup>You are here</Popup>
            </Marker>
        ) : null;
        return (
            <Map center={this.props.center}
                 zoom={10}
                 onClick={this.handleClick}
                 onLocationfound={this.handleLocationFound}
                 id="mapid">
                <TileLayer
                    attribution=''
                    url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />
                {marker}
            </Map>)
    }
}