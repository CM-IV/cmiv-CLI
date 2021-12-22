import { Fragment } from "preact";

const Wrapper = (props: any) => {
  return (
    <Fragment>
      <div class="container">
        <div class="columns">
          <div class="column is-full">{props.children}</div>
        </div>
      </div>
    </Fragment>
  );
};

export { Wrapper };
