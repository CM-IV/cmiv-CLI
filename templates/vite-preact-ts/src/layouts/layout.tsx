import { Wrapper } from "../components/wrapper";
import { Nav } from "../components/nav";

const Layout = (props: any) => {
  return (
    <Wrapper>
      <Nav />
      {props.children}
    </Wrapper>
  );
};

export { Layout };