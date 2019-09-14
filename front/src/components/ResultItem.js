import React, {Component} from 'react'
import { Message } from 'semantic-ui-react'

export class ResultItem extends Component<> {


    render() {
        return (<Message style={{backgroundColor: "#" + this.props.color.toString(16)}}>
            <Message.Header>{this.props.title}</Message.Header>
            {this.props.description}
        </Message>)
    }
}

export default class ResultItemsList extends Component<> {


    render() {
        return this.props.schedule.map((x, i) =>
                        <ResultItem
                            key={i}
                            title={x.name}
                            description={x.description}
                            color={x.color}/>
                    )
    }
}
