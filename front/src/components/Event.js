import React, {Component} from 'react';
import {Input, Dropdown, Button, Icon} from 'semantic-ui-react'
import {OptionsContext, CenterContext} from '../lib/api'
import '../App.css'
import CoordinatesSelector from './CoordinatesSelector'


const eventTypes = [
    {
        text: "Fixed place",
        key: 1,
        value: "fixed_place"
    },
    {
        text: "Category",
        key: 2,
        value: "category"
    }
];

class FixedEvent extends Component {
    onTextChange = (e) => {
        const value = e.target.value;
        const newEventState = {
            name: value,
        };
        this.props.onChange(newEventState);
    };

    render() {
        return (
            <CenterContext.Consumer>
                {center =>
                    <React.Fragment>
                        <Input placeholder="Place name"
                               fluid
                               defaultValue={this.props.event.name}
                               onChange={this.onTextChange}/>
                        <CoordinatesSelector center={center}
                                             onChange={this.props.onChange}
                                             location={this.props.event.location}/>
                    </React.Fragment>}
            </CenterContext.Consumer>
        )
    }
}


class CategoryEvent extends Component {

    // liftState = () => {
    //     this.props.onChange({
    //         type: this.state.category,
    //         brand: this.state.brand
    //     });
    // };

    onChangeCategory = (e, data) => {
        const value = data.value;
        this.props.onChange({
            category: value,
        });
    };

    onChangeBrand = (e, data) => {
        const value = data.value;
        this.props.onChange({
            brand: value,
        });
    };


    render() {
        return (<OptionsContext.Consumer>
            {contextOptions =>
                <React.Fragment>
                    <Dropdown placeholder='Category'
                              defaultValue={this.props.event.category}
                              onChange={this.onChangeCategory}
                              fluid search selection
                              options={contextOptions.categoriesList}/>
                    {this.props.event.category &&
                    <Dropdown clearable
                              placeholder='Brand (optional)'
                              defaultValue={this.props.event.brand}
                              onChange={this.onChangeBrand}
                              fluid search selection
                              options={contextOptions.brands[this.props.event.category]}/>}
                </React.Fragment>}

        </OptionsContext.Consumer>)
    }
}

export class Event extends Component {

    renderEvent = (event) => {
        switch (event.type) {
            case 'fixed_place':
                return (<FixedEvent event={event}
                                    onChange={this.props.onChange}/>);
            case 'category':
                return (<CategoryEvent event={event}
                                       onChange={this.props.onChange}/>);
            default:
                return null
        }
    };
    onChangeType = (e, data) => {
        const value = data.value;
        this.props.onChange({type: value});
    };


    render() {
        const event = this.renderEvent(this.props.event);
        return (
            <div className="ui raised segment">
                <Button primary className="icon" onClick={this.props.down}>
                    <Icon name="caret down"/>
                </Button>
                <Button primary className="icon" onClick={this.props.up}>
                    <Icon name="caret up"/>
                </Button>
                <Button color="red" className="icon" onClick={this.props.drop}>
                    <Icon name="delete"/>
                </Button>
                <br/>
                <Dropdown clearable
                          placeholder='Event type'
                          defaultValue={this.props.event.type}
                          onChange={this.onChangeType}
                          fluid selection options={eventTypes}/>
                {event}
            </div>
        );
    }
}