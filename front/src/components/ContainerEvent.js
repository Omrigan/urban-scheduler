import {Button, Icon, Tab} from "semantic-ui-react";
import update from "immutability-helper";
import React, {Component} from "react";
import {Event} from "./Event";

function get_empty() {
    return {type: null};
}

export default class ContainerEvent extends Component {

    renderEvent = (subevent, idx) => {
        const event = <Event key={idx}
                             event={subevent}
                             down={this.childDown.bind(idx)}
            // up={this.childUp.bind(this, i)}
                             drop={this.childDrop.bind(idx)}
                             onChange={this.childUpdate.bind(this, idx)}/>;
        return event;
    };

    render() {

        let objects;
        if (this.props.event.type === 'parallel') {
            const panes = this.props.event.items.map((subevent, idx) => (
                {
                    menuItem: idx,
                    render: () => <Tab.Pane>
                        {this.renderEvent(subevent, idx)}
                    </Tab.Pane>
                }
            ));
            objects = [<Tab key="tab" panes={panes}/>];
        } else {
            objects = this.props.event.items.map(this.renderEvent);
        }
        return <React.Fragment>
            <Button color="green" className="icon" onClick={this.addChild}>
                <Icon name="plus"/>
            </Button>
            {objects}
        </React.Fragment>;
    }

    // Event manipulation
    addChild = () => {
        console.log("Adding")
        this.propagateChanges({$push: [get_empty()]});

    };

    childDown = (key) => {
        const items = this.props.event.items;
        if ((key + 1) == items.length) {
            return;
        }
        const head = items.slice(0, key);
        const tail = items.slice(key + 2, items.length);
        const result = head.concat([items[key + 1], items[key]]).concat(tail);

        this.propagateChanges(result);
    };

    // childUp = (key) => {
    //     this.setState(state => {
    //         if (key == 0) {
    //             return state;
    //         }
    //         const head = state.eventStates.slice(0, key - 1);
    //         const tail = state.eventStates.slice(key + 1, state.eventStates.length);
    //         const result = head.concat([state.eventStates[key],
    //             state.eventStates[key - 1]]).concat(tail);
    //
    //         return {
    //             eventStates: result
    //         };
    //     })
    // };

    childDrop = (key) => {
        this.propagateChanges(this.props.event.items.filter((value, idx) => (idx != key)));
    };

    childUpdate = (key, newContent, erase) => {
        let eventMutator;
        if (erase) {
            eventMutator = {$set: newContent};
        } else {
            eventMutator = {$merge: newContent};
        }
        let mutator = {};
        mutator[key] = eventMutator;
        this.propagateChanges(mutator);
    };


    propagateChanges = (newItems) => {
        this.props.onChange({items: update(this.props.event.items, newItems)});
    }

}
