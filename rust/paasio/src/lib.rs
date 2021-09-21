use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    nreads: usize,
    nbytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped: wrapped,
            nreads: 0,
            nbytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.nbytes
    }

    pub fn reads(&self) -> usize {
        self.nreads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let nbytes = self.wrapped.read(buf)?;
        self.nreads += 1;
        self.nbytes += nbytes;

        Ok(nbytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    nwrites: usize,
    nbytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped: wrapped,
            nwrites: 0,
            nbytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.nbytes
    }

    pub fn writes(&self) -> usize {
        self.nwrites
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let nbytes = self.wrapped.write(buf)?;
        self.nwrites += 1;
        self.nbytes += nbytes;

        Ok(nbytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()?;
        Ok(())
    }
}
