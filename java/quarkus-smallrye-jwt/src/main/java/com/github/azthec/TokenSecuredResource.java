package com.github.azthec;

import jakarta.annotation.security.PermitAll;
import jakarta.annotation.security.RolesAllowed;
import jakarta.enterprise.context.RequestScoped;
import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.InternalServerErrorException;
import jakarta.ws.rs.NotFoundException;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.SecurityContext;

import org.eclipse.microprofile.jwt.JsonWebToken;
import org.jboss.resteasy.reactive.RestPath;
import org.jboss.resteasy.reactive.RestResponse;
import org.jboss.resteasy.reactive.server.ServerExceptionMapper;

import io.quarkus.security.UnauthorizedException;
import io.smallrye.mutiny.Uni;

/*
 * This dummy path tests role based authentication, with the extra validation
 * that the user role can depend on the path. In this case the the RoleMatchingLabel
 * checks that the user has a role which exactly matches the label. This validation
 * is enabled with the custom annotation @RoleMatchingLabel
 */
@Path("/secured/{label}")
@RequestScoped
public class TokenSecuredResource {

  @Inject
  JsonWebToken jwt;

  @GET
  @Path("dummy-endpoint")
  public Uni<RestResponse<String>> restEndpoint(@RestPath String label) {
    return Uni.createFrom().item("Dummy endpoint returns this string").map(RestResponse::ok);
  }

  @GET
  @Path("permit-all")
  @PermitAll
  @Produces(MediaType.TEXT_PLAIN)
  public String hello(@Context SecurityContext ctx) {
    return getResponseString(ctx);
  }

  @GET
  @Path("roles-allowed")
  @Produces(MediaType.TEXT_PLAIN)
  @RoleMatchingLabel
  public String helloRolesAllowed(@Context SecurityContext ctx) {
    return getResponseString(ctx);
  }

  @GET
  @Path("roles-allowed-admin")
  @RolesAllowed("admin")
  @Produces(MediaType.TEXT_PLAIN)
  public String helloRolesAllowedAdmin(@Context SecurityContext ctx) {
    return getResponseString(ctx);
  }

  private String getResponseString(SecurityContext ctx) {
    String name;
    if (ctx.getUserPrincipal() == null) {
      name = "anonymous";
    } else if (!ctx.getUserPrincipal().getName().equals(jwt.getName())) {
      throw new InternalServerErrorException("Principal and JsonWebToken names do not match");
    } else {
      name = ctx.getUserPrincipal().getName();
    }
    return String.format("hello + %s,"
        + " isHttps: %s,"
        + " authScheme: %s,"
        + " hasJWT: %s",
        name, ctx.isSecure(), ctx.getAuthenticationScheme(), hasJwt());
  }

  private boolean hasJwt() {
    return jwt.getClaimNames() != null;
  }

  @ServerExceptionMapper
  public RestResponse<String> mapException(Throwable ex) {
    var status = (RestResponse.Status) null;

    if (ex instanceof NotFoundException) {
      status = RestResponse.Status.NOT_FOUND;

    } else if (ex instanceof IllegalArgumentException) {
      status = RestResponse.Status.BAD_REQUEST;

    } else if (ex instanceof UnauthorizedException) {
      status = RestResponse.Status.UNAUTHORIZED;

    } else {
      status = RestResponse.Status.INTERNAL_SERVER_ERROR;
    }

    return RestResponse.status(status, ex.getMessage());
  }
}
