--
-- Copyright (c) 2020-2021 science+computing ag and other contributors
--
-- This program and the accompanying materials are made
-- available under the terms of the Eclipse Public License 2.0
-- which is available at https://www.eclipse.org/legal/epl-2.0/
--
-- SPDX-License-Identifier: EPL-2.0
--

-- Your SQL goes here
ALTER TABLE
    artifacts
ADD COLUMN
    job_id INTEGER REFERENCES jobs(id) NOT NULL
