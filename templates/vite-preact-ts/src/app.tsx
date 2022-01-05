import { Route, Switch } from "wouter-preact";
import { Home } from "./pages/Home";
import { NotFound } from "./pages/_404";


const App = () => {

  return (

    <div class="app">
      <Switch>
        <Route path="/" component={Home}></Route>
        <Route component={NotFound}></Route>
      </Switch>
    </div>

  )

}

export { App };