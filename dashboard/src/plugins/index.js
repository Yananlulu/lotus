import nut from './nut'
import forum from './forum'
import market from './market'

const plugins = [nut, forum, market]

export default {
  menus: plugins.reduce((ar, it) => {
    return ar.concat(it.menus)
  }, []),
  routes: plugins.reduce((ar, it) => {
    return ar.concat(it.routes)
  }, [])
}
