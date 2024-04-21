package com.github.azthec;

import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;

@QuarkusTest
class TokenSecuredResourceTest {
  @Test
  void testHelloEndpoint() {
    given()
        .when().get("/secured/user-label/roles-allowed")
        .then()
        .statusCode(403);
  }
}
