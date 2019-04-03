import React, {Component} from 'react';
import '../App.css';
import ListOfMarkers from './ListOfMarkers';
import ResultItemsList from './ResultItem';

import Report from './Report';


export class Result extends Component {


    render() {
        return (<div>
            <Report report={this.props.result.report}/>

            <ResultItemsList schedule={this.props.result.schedule}/>

            <ListOfMarkers schedule={this.props.result.schedule}
                           center={this.props.result.center}/>
        </div>)
    }
}

