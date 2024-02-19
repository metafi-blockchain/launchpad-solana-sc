import { Container, Row, Col } from "react-bootstrap"
import { Link } from "react-router-dom"

const Error403 = () => {
    return <div className="hb-not-found py-6">
        <Container>
            <Row className="justify-content-center">
                <Col xl={7} lg={10} className="px-4">
                    <h1>403</h1>
                    <h4 className="mb-4">Oops! This Page Could Not Be Found</h4>
                    <p className="mb-5">Sorry but the page you are looking for does not exist, <br className="d-none d-md-block" />have been removed. name changed or is temporarily unavailable</p>
                    <div>
                        <Link className="btn btn-primary" to={"/"}>Back to Homepage</Link>
                    </div>
                </Col>
            </Row>
        </Container>
    </div>
}
export default Error403