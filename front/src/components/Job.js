import {AddButton} from "./Event";
// import {Button} from "reactstrap";
import {Button, Label, Icon} from "semantic-ui-react";
import React, {Component} from 'react';
import {getCities, getOptions, postProblem} from "../lib/api";
import {OptionsContext, CenterContext} from "../lib/api";
import update from 'immutability-helper';
import Config from "./Config";
import {exportProblem, importProblem, saveProblem, loadProblem} from "../lib/ioManager";

import {safeLoad, safeDump} from "js-yaml"
import ContainerEvent from "./ContainerEvent";


function empty_container() {
    return {items: []}
}

const defaultProblemVersion = 2;

export class Job extends Component {
    constructor(props) {
        super(props);

        this.state = {
            eventContainer: empty_container(),
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
            });
        }).then(() =>
            this.setProblem(loadProblem() || empty_container())
        );


        getCities(this.setState.bind(this));
    }

    changeContainer = (newContent) => {
        this.setState((state) => {
            return {
                eventContainer: newContent
            }
        });
    };

    changeConfig = (newConfig) => {
        this.setState((state) =>
            ({config: update(state.config, {$merge: newConfig})}))
    };


    getProblem = (version) => {
        if (!version)
            version = defaultProblemVersion;
        let problem = {
            config: this.state.config,
            version: version
        };

        if (version === 1) {
            problem.ordered_events = this.state.eventContainer.items;
        } else {
            problem.events = this.state.eventContainer.items;
        }
        return problem;
    };

    setProblem = (problem) => {
        this.setState({
            config: problem.config,
            eventContainer: {items: problem.events},
        })
    };


    legacySend = () => {
        const noLegacy = this.state.eventContainer.items.some((event) =>
            (event.type === "parallel" || event.type === "sequential"));
        if (noLegacy) {
            this.props.updateError({error_message: "No nested events!"})
        }

        this.props.startPredict();
        const problem = this.getProblem(1);
        postProblem(problem, this.props.updateResult, this.props.updateError);
    };

    send = () => {
        this.props.startPredict();
        const problem = this.getProblem(2);
        postProblem(problem, this.props.updateResult, this.props.updateError);
    };


    save = () => {
        saveProblem(this.getProblem());
        this.props.saveResult();
    };

    exportProblemBtn = () => {
        exportProblem(this.getProblem());
    };

    importProblemBtn = (e) => {
        this.setProblem(importProblem(e));
    };

    clear = () => {
        this.setState({eventContainer: empty_container()})
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
                <Config onChangeConfig={this.changeConfig}
                        config={this.state.config}
                        cities={this.state.cities}/>

                <Button color='green' onClick={this.send}>Send</Button>
                <Button color='yellow' onClick={this.legacySend}>Legacy send</Button>
                <AddButton onChange={this.changeContainer}
                           event={this.state.eventContainer}/>
                <br/><br/>
                <Button color='orange'
                        onClick={this.save}>Save</Button>
                <Button color='teal'
                        onClick={this.clear}>Clear</Button>
                <Button color='purple'
                        onClick={this.exportProblemBtn}>Export</Button>
                <Button as="label" basic htmlFor="upload">
                    Import
                    <input onChange={this.importProblemBtn}
                           hidden
                           id="upload"
                           multiple
                           type="file"
                    />
                </Button>


                <CenterContext.Provider value={this.getCenter()}>
                    <OptionsContext.Provider value={this.state.options}>
                        <ContainerEvent onChange={this.changeContainer}
                                        event={this.state.eventContainer}/>
                    </OptionsContext.Provider>
                </CenterContext.Provider>
            </div>
        );
    }
}