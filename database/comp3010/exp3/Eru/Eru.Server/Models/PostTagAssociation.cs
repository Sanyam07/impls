﻿using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations.Schema;
using System.Linq;
using System.Threading.Tasks;

namespace Eru.Server.Models
{
    public class PostTagAssociation
    {
        public int TagId { get; set; }
        public int PostId { get; set; }
        [ForeignKey("PostId")] public Post Post { get; set; }
        [ForeignKey("TagId")] public PostTag Tag { get; set; }
    }
}