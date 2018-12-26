import React, {Component} from 'react'
import {Link} from 'react-router-dom'

class Widget extends Component {
  render() {
    return (<div>
      <Link to="/users/sign-in">sign in</Link>
      <Link to="/users/sign-up">sign up</Link>
    </div>)
  }
}

export default Widget
