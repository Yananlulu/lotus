import React from 'react'
import Loadable from 'react-loadable'
import {Spin} from 'antd'

const wrapper = (loader) => {
  const W = Loadable({
    loader,
    loading: () => (<Spin size="large"/>)
  })
  return() => (<W/>)
}

export default wrapper
