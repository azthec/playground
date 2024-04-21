package com.github.azthec;

import io.quarkus.test.junit.QuarkusTest;
import io.quarkus.test.security.TestSecurity;
import io.quarkus.test.security.jwt.JwtSecurity;

import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;

@QuarkusTest
class TokenSecuredResourceTest {
  @Test
  void testPermitAll() {
    given()
        .when().get("/secured/all/permit-all")
        .then()
        .statusCode(200);
  }

  @Test
  void testUnauthenticatedUser() {
    given()
        .when().get("/secured/user/roles-allowed")
        .then()
        .statusCode(403);
  }

  @Test
  @TestSecurity(user = "dummy")
  @JwtSecurity
  void testAutenticatedMissingRole() {
    given()
        .when().get("/secured/user/roles-allowed")
        .then()
        .statusCode(403);
  }

  @Test
  @TestSecurity(user = "dummy", roles = { "user" })
  @JwtSecurity
  void testAuthenticatedUser() {
    given()
        .when().get("/secured/user/roles-allowed")
        .then()
        .statusCode(200);
  }

  @Test
  @TestSecurity(user = "dummy", roles = { "user" })
  @JwtSecurity
  void testAuthenticatedUserWrongRole() {
    given()
        .when().get("/secured/admin/roles-allowed-admin")
        .then()
        .statusCode(403);
  }

  @Test
  @TestSecurity(user = "dummy", roles = { "admin" })
  @JwtSecurity
  void testAuthenticatedAdmin() {
    given()
        .when().get("/secured/admin/roles-allowed-admin")
        .then()
        .statusCode(200);
  }
}
