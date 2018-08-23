import nut from './nut'
import forum from './forum'
import market from './market'

const plugins = {
  nut,
  forum,
  market
}

export default {
  menus: plugins.reduce((ar, it) => {
    ar.concat(it.menus)
    return ar
  }, []),
  routes: plugins.reduce((ar, it) => {
    ar.concat(it.routes)
    return ar
  }, [])
}