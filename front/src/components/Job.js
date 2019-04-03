import {Event} from "./Event";
// import {Button} from "reactstrap";
import {Button} from "semantic-ui-react";
import React, {Component} from 'react';
import {getOptions, OptionsContext, postJob} from "../lib/api";
import update from 'immutability-helper';
import Config from "./Config";


const startEvents = 5;

function get_empty() {
    return {type: null};
}

const pureState = [...Array(startEvents)].map(() => (get_empty()));

export class Job extends Component {
    loadState = () => {
        let loadedState = localStorage.getItem('eventStates');
        if (loadedState) {
            loadedState = JSON.parse(loadedState);
            this.setState({eventStates: loadedState})
        }
    };

    constructor(props) {
        super(props);

        this.state = {
            eventStates: [],
            options: {},
            config: {
                routingBackend: "dummy",
                clipping: null,
            }
        };

        getOptions((options) => {
            this.setState({options: options});
            this.loadState();
        });
        this.addEvent = this.addEvent.bind(this);
    }


    eventChanged = (key, newContent) => {
        // this.setState({
        //     eventStates: this.state.eventStates.map((x, i) =>
        //         (i === key ? Object.assign(x, newContent) : x))
        // });
        this.setState((state) => {
            const myobj = {};
            myobj[key] = {$merge: newContent};

            return update(state, {
                eventStates: myobj
            })
        });


    };
    onChangeConfig = (newConfig) => {
        this.setState((state) =>
            ({config: update(state.config,{$merge: newConfig})}))
    };

    addEvent = () => {
        this.setState(state => ({
            eventStates: state.eventStates.concat([get_empty()])
        }))
    };

    send = () => {
        const job = {
            ordered_events: this.state.eventStates,
            config: this.state.config
        };
        postJob(job, this.props.updateResult);

    };
    save = () => {
        localStorage.setItem('eventStates', JSON.stringify(this.state.eventStates))
    };

    clear = () => {
        this.setState({eventStates: pureState})
    };

    render() {
        return (
            <div className="">
                <Config onChangeConfig={this.onChangeConfig}
                        config={this.state.config}/>

                <Button color='green' onClick={this.send}>Send</Button>
                <Button primary onClick={this.addEvent}>Add</Button>
                <Button color='orange'
                        onClick={this.save}>Save</Button>
                <Button color='teal'
                        onClick={this.clear}>Clear</Button>

                <OptionsContext.Provider value={this.state.options}>
                    {this.state.eventStates.map((x, i) =>
                        <Event key={i.toString()}
                               event={x}
                               onChange={this.eventChanged.bind(this, i)}/>
                    )} <br/>

                </OptionsContext.Provider>
            </div>
        );
    }
}