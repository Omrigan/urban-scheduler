import React, {Component} from 'react';
import {Input, Dropdown, Button, Icon, Tab} from 'semantic-ui-react'
import ContainerEvent from "./ContainerEvent";

import {OptionsContext, CenterContext} from '../lib/api'
import '../App.css'
import CoordinatesSelector from './CoordinatesSelector'
import update from "immutability-helper";

function get_empty() {
    return {type: null};
}

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
    },
    {
        text: "Parallel",
        key: 3,
        value: "parallel"
    },
    {
        text: "Sequential",
        key: 4,
        value: "sequential"
    },
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
            name: value
        });

    };

    onChangeBrand = (e, data) => {
        const value = data.value;
        if (value == "") {
            const cat = this.props.event.category;
            this.props.onChange({
                category: cat,
                name: cat
            })
        } else {
            this.props.onChange({
                brand: value,
                name: value
            });
        }
    };


    render() {
        console.log("Context consumer", OptionsContext.Consumer);
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

export class AddButton extends Component {
    onClick = () => {
        const mutator = {$push: [get_empty()]};
        this.props.onChange({items: update(this.props.event.items, mutator)});
    };

    render() {
        return <Button color="green" className="icon" onClick={this.onClick}>
            <Icon name="plus"/>
        </Button>;
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
            case 'parallel':
                return (<ContainerEvent event={event}
                                        onChange={this.props.onChange}/>);
            case 'sequential':
                return (<ContainerEvent event={event}
                                        onChange={this.props.onChange}/>);
            default:
                return null
        }
    };

    onChangeType = (e, data) => {
        const value = data.value;
        let result = {
            type: value,
            name: value
        };
        if (value == 'sequential' || value == 'parallel') {
            result.items = this.props.event.items || [];
        }
        this.props.onChange(result, true);
    };


    render() {
        const event = this.renderEvent(this.props.event);
        const canAdd = this.props.event.type === "parallel" ||
            this.props.event.type === "sequential";
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
                {canAdd && <AddButton onChange={this.props.onChange}
                                      event={this.props.event}/>}
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