import React from 'react'
import importedComponent from 'react-imported-component'
import { Redirect, Route, Switch } from 'react-router'
import { BrowserRouter } from 'react-router-dom'
import { Provider as MobxProvider } from 'mobx-react'
import { Provider as KeepAliveProvider } from 'react-keep-alive'

const asyncComponentFactory = (resolve: () => Promise<React.ComponentType<any> | { default: React.ComponentType<any> }>) =>
  importedComponent(resolve)

const UserView = asyncComponentFactory(() => import('./views/User'))

const App: React.FC = () => {
  return (
    <MobxProvider>
      <KeepAliveProvider>
        <BrowserRouter>
          <Switch>
            <Route exact path='/' component={UserView}/>
            <Redirect from='/*' to='/error'/>
          </Switch>
        </BrowserRouter>
      </KeepAliveProvider>
    </MobxProvider>
  )
}

export default App
