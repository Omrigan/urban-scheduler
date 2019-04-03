import React, {Component} from 'react'
import { Message } from 'semantic-ui-react'

export class ResultItem extends Component<> {


    render() {
        return (<Message>
            <Message.Header>{this.props.title}</Message.Header>
            {this.props.description}
        </Message>)
    }
}

export default class ResultItemsList extends Component<> {


    render() {
        return this.props.schedule.map((x, i) =>
                        <ResultItem title={x.name} description={x.description}/>
                    )
    }
}
