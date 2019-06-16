import {Event} from "./Event";
// import {Button} from "reactstrap";
import {Button, Label, Icon} from "semantic-ui-react";
import React, {Component} from 'react';
import {getCities, getOptions, OptionsContext, CenterContext, postJob} from "../lib/api";
import update from 'immutability-helper';
import Config from "./Config";
import {saveEventStates, loadEventStates} from "../lib/localstorageManager";
import {safeLoad, safeDump} from "js-yaml"


const startEvents = 0;

function get_empty() {
    return {type: null};
}

const pureState = [...Array(startEvents)].map(() => (get_empty()));

export class Job extends Component {


    constructor(props) {
        super(props);

        this.state = {
            eventStates: [],
            options: {},
            config: {
                routingBackend: "dummy",
                clipping: null,
                solver: "rust",
                city: "moscow"
            },
            cities: [{
                text: 'moscow',
                key: 'moscow',
                value: 'moscow'
            }],
            citiesRaw: {
                moscow: {
                    radius: 20000,
                    center: [55.7494539, 37.62160470000001]
                },
            },
        };

        getOptions((options) => {
            this.setState({
                options: options,
                eventStates: loadEventStates()
            });

        });


        getCities(this.setState.bind(this));

        this.addEvent = this.addEvent.bind(this);
    }


    eventChanged = (key, newContent, erase) => {
        // this.setState({
        //     eventStates: this.state.eventStates.map((x, i) =>
        //         (i === key ? Object.assign(x, newContent) : x))
        // });
        this.setState((state) => {
            const myobj = {};
            if (erase) {
                myobj[key] = {$set: newContent};
            } else {
                myobj[key] = {$merge: newContent};
            }

            return update(state, {
                eventStates: myobj
            })
        });


    };
    onChangeConfig = (newConfig) => {
        this.setState((state) =>
            ({config: update(state.config, {$merge: newConfig})}))
    };

    addEvent = () => {
        this.setState(state => ({
            eventStates: state.eventStates.concat([get_empty()])
        }))
    };


    send = () => {
        this.props.startPredict();
        const job = {
            ordered_events: this.state.eventStates,
            config: this.state.config
        };
        postJob(job, this.props.updateResult);

    };

    exportProblemFile = () => {
        const element = document.createElement("a");
        const file = new Blob([safeDump(this.state.eventStates)], {type: 'plain/text'});
        element.href = URL.createObjectURL(file);
        element.download = "problem.yaml";
        document.body.appendChild(element); // Required for this to work in FireFox
        element.click();
    };
    importProblemFile = (e) => {
        let file = e.target.files[0];
        if (!file) {
            return;
        }
        let reader = new FileReader();
        reader.onload = (e) => {
            let contents = safeLoad(e.target.result);
            console.log(contents);
            // Display file content
            this.setState({eventStates: contents.ordered_events})
        };
        reader.readAsText(file);
    };


    clear = () => {
        this.setState({eventStates: pureState})
    };

    save = () => {
        saveEventStates(this.state.eventStates);
        this.props.saveResult();
    };

    getCenter = () => {
        if (this.state.config.city) {
            return this.state.citiesRaw[this.state.config.city].center
        } else {
            return ([0, 0])
        }
    };

    render() {
        return (
            <div className="">
                <Config onChangeConfig={this.onChangeConfig}
                        config={this.state.config}
                        cities={this.state.cities}/>

                <Button color='green' onClick={this.send}>Send</Button>
                <Button primary onClick={this.addEvent}>Add</Button>
                <Button color='orange'
                        onClick={this.save}>Save</Button>
                <Button color='teal'
                        onClick={this.clear}>Clear</Button>
                <Button color='purple'
                        onClick={this.exportProblemFile}>Export</Button>
                <Button
                    as="label"
                    basic
                    htmlFor="upload"
                >
                    Import
                    <input onChange={this.importProblemFile}
                           hidden
                           id="upload"
                           multiple
                           type="file"
                    />
                </Button>
                <CenterContext.Provider value={this.getCenter()}>
                    <OptionsContext.Provider value={this.state.options}>
                        {this.state.eventStates.map((x, i) =>
                            <Event key={i.toString()}
                                   event={x}
                                   onChange={this.eventChanged.bind(this, i)}/>
                        )} <br/>

                    </OptionsContext.Provider>
                </CenterContext.Provider>
            </div>
        );
    }
}