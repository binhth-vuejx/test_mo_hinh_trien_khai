FROM oven/bun:1.1.18 as base
WORKDIR /app
ADD . .
FROM oven/bun:1.1.18 AS release
WORKDIR /app
RUN bun add hono

COPY --from=base /app /app
ENV NODE_ENV=production
ENV TZ="Asia/Ho_Chi_Minh"
RUN chmod +x /app/init.sh
ENTRYPOINT ["sh", "/app/init.sh"]