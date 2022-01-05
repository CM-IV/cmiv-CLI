import { Fragment } from "preact";

const Wrapper = (props: any) => {
  return (
    <Fragment>
      <div class="container">
        <div class="columns is-centered">
          <div class="column is-four-fifths">{props.children}</div>
        </div>
      </div>
    </Fragment>
  );
};

export { Wrapper };