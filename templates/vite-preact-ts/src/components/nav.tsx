import { Fragment } from "preact";
import { Link } from "wouter-preact";

const Nav = () => {
  return (
    <Fragment>
      <nav
        class="navbar"
        id="nav"
        role="navigation"
        aria-label="main navigation"
      >
        <div class="navbar-brand">
          <a class="image is-48x48 mt-1" href="/">
            <img
              src="https://ik.imagekit.io/xbkhabiqcy9/img/Occident_Tech_logo_wyOOeI6Bhcn.png?updatedAt=1637081403933"
              height="48"
              width="48"
              class="image-responsive"
              alt="logo"
            />
          </a>
        </div>

        <input
          type="checkbox"
          id="navbar-burger-toggle"
          class="navbar-burger-toggle is-hidden"
        />
        <label for="navbar-burger-toggle" class="navbar-burger">
          <span></span>
          <span></span>
          <span></span>
        </label>

        <div id="navMenu" class="navbar-menu">
          <div class="navbar-start">
            <Link to={"/"} class="navbar-item mt-2">
              Home
            </Link>
          </div>
        </div>
      </nav>
    </Fragment>
  );
};

export { Nav };

