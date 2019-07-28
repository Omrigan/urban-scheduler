import {Button, Icon, Tab} from "semantic-ui-react";
import update from "immutability-helper";
import React, {Component} from "react";
import {Event} from "./Event";


export default class ContainerEvent extends Component {

    renderEvent = (subevent, idx) => {
        const event = <Event key={idx}
                             event={subevent}
                             down={this.childDown.bind(this, idx)}
                             up={this.childUp.bind(this, idx)}
                             drop={this.childDrop.bind(this, idx)}
                             onChange={this.childUpdate.bind(this, idx)}/>;
        return event;
    };

    render() {

        let objects;
        if (this.props.event.type === 'parallel') {
            const panes = this.props.event.items.map((subevent, idx) => (
                {
                    menuItem: subevent.name,
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

            {objects}
        </React.Fragment>;
    }

    // Event manipulation

    childDown = (key) => {
        const items = this.props.event.items;
        if ((key + 1) == items.length) {
            return;
        }
        const head = items.slice(0, key) || [];
        const tail = items.slice(key + 2, items.length) || [];
        const result = head.concat([items[key + 1], items[key]]).concat(tail);
        console.log(result);
        this.propagateChanges({$set: result});
    };

    childUp = (key) => {
        if (key == 0) {
            return;
        }
        this.childDown(key-1);
    };



    childDrop = (key) => {
        this.propagateChanges({
            $set: this.props.event.items.filter((value, idx) => (idx != key))
        });
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
