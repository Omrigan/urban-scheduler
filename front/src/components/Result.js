import React, {Component} from 'react';
import '../App.css';
import ListOfMarkers from './ListOfMarkers';
import ResultMap from './ResultMap'
import ResultItemsList from './ResultItem';

import Report from './Report';


export class Result extends Component {


    render() {
        return (<div>
            {this.props.result.report && <Report report={this.props.result.report}/>}

            <ResultItemsList schedule={this.props.result.schedule}/>

            <ResultMap schedule={this.props.result.schedule}
                       final_route={this.props.result.final_route}
                       center={this.props.result.center}/>
        </div>)
    }
}

