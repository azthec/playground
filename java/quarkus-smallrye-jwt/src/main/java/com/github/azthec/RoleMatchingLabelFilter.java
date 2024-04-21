package com.github.azthec;

import jakarta.annotation.Priority;
import jakarta.ws.rs.Priorities;
import jakarta.ws.rs.container.ContainerRequestContext;
import jakarta.ws.rs.container.ContainerRequestFilter;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.SecurityContext;
import jakarta.ws.rs.core.UriInfo;
import jakarta.ws.rs.ext.Provider;

@Provider
@Priority(Priorities.AUTHORIZATION)
@RoleMatchingLabel
public class RoleMatchingLabelFilter implements ContainerRequestFilter {

  @Context
  private UriInfo uriInfo;

  @Context
  private SecurityContext securityContext;

  @Override
  public void filter(ContainerRequestContext requestContext) {
    String label = uriInfo.getPathParameters().getFirst("label");
    if (label != null && !securityContext.isUserInRole(label)) {
      requestContext
          .abortWith(Response.status(Response.Status.FORBIDDEN).entity("User not authorized for this role").build());
    }
  }
}
